macro_rules! deps {
    () => {
        AssertNotLoaded!();
        LoadResult!();
        NotLoaded!();
        LoadDepGraph!();
        AssertLoaded!();
        Ok!();
        DeleteIncompatible!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < T : Default > LoadResult < T > { # [doc = " Accesses the data returned in [`LoadResult::Ok`]."] pub fn open (self , sess : & Session) -> T { match (sess . opts . assert_incr_state , & self) { (Some (IncrementalStateAssertion :: NotLoaded) , LoadResult :: Ok { .. }) => { sess . dcx () . emit_fatal (errors :: AssertNotLoaded) ; } (Some (IncrementalStateAssertion :: Loaded) , LoadResult :: LoadDepGraph (..) | LoadResult :: DataOutOfDate ,) => { sess . dcx () . emit_fatal (errors :: AssertLoaded) ; } _ => { } } ; match self { LoadResult :: LoadDepGraph (path , err) => { sess . dcx () . emit_warn (errors :: LoadDepGraph { path , err }) ; Default :: default () } LoadResult :: DataOutOfDate => { if let Err (err) = delete_all_session_dir_contents (sess) { sess . dcx () . emit_err (errors :: DeleteIncompatible { path : dep_graph_path (sess) , err }) ; } Default :: default () } LoadResult :: Ok { data } => data , } } }
    };
}

impl_140!()