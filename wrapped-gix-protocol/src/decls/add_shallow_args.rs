macro_rules! deps {
    () => {
        Error!();
        Arguments!();
        Shallow!();
    };
}

macro_rules! add_shallow_args {
    () => {
        deps!();
        fn add_shallow_args (args : & mut Arguments , shallow : & Shallow , shallow_file : & std :: path :: Path ,) -> Result < (Option < Vec < gix_hash :: ObjectId > > , Option < gix_lock :: File >) , Error > { let expect_change = * shallow != Shallow :: NoChange ; let shallow_lock = expect_change . then (| | acquire_shallow_lock (shallow_file)) . transpose () ? ; let shallow_commits = gix_shallow :: read (shallow_file) ? ; if (shallow_commits . is_some () || expect_change) && ! args . can_use_shallow () { return Err (Error :: MissingServerFeature { feature : "shallow" , description : "shallow clones need server support to remain shallow, otherwise bigger than expected packs are sent effectively unshallowing the repository" , }) ; } if let Some (shallow_commits) = & shallow_commits { for commit in shallow_commits . iter () { args . shallow (commit) ; } } match shallow { Shallow :: NoChange => { } Shallow :: DepthAtRemote (commits) => args . deepen (commits . get () as usize) , Shallow :: Deepen (commits) => { args . deepen (* commits as usize) ; args . deepen_relative () ; } Shallow :: Since { cutoff } => { args . deepen_since (cutoff . seconds) ; } Shallow :: Exclude { remote_refs , since_cutoff , } => { if let Some (cutoff) = since_cutoff { args . deepen_since (cutoff . seconds) ; } for ref_ in remote_refs { args . deepen_not (ref_ . as_ref () . as_bstr ()) ; } } } Ok ((shallow_commits , shallow_lock)) }
    };
}

add_shallow_args!()