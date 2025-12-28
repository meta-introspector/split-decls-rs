macro_rules! deps {
    () => {
        Action!();
        ChangeRef!();
        Error!();
        Tracker!();
    };
}

macro_rules! emit_addition {
    () => {
        deps!();
        fn emit_addition < 'rhs , 'lhs : 'rhs , E > ((idx , path , entry) : (usize , & 'rhs BStr , & 'rhs gix_index :: Entry) , mut cb : impl FnMut (ChangeRef < 'lhs , 'rhs >) -> Result < Action , E > , tracker : Option < & mut rewrites :: Tracker < ChangeRef < 'lhs , 'rhs > > > ,) -> Result < Action , Error > where E : Into < Box < dyn std :: error :: Error + Send + Sync > > , { if ignore_unmerged_and_intent_to_add ((idx , path , entry)) { return Ok (Action :: Continue) ; } let change = ChangeRef :: Addition { location : Cow :: Borrowed (path) , index : idx , entry_mode : entry . mode , id : Cow :: Borrowed (entry . id . as_ref ()) , } ; let change = match tracker { None => change , Some (tracker) => match tracker . try_push_change (change , path) { Some (change) => change , None => return Ok (Action :: Continue) , } , } ; cb (change) . map_err (| err | Error :: Callback (err . into ())) }
    };
}

emit_addition!();