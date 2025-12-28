macro_rules! deps {
    () => {
        CandidateInfo!();
        Tag!();
        Commit!();
        Object!();
    };
}

macro_rules! impl_876 {
    () => {
        deps!();
        impl std :: fmt :: Display for CandidateInfo { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { CandidateInfo :: FindError { source } => write ! (f , "lookup error: {source}") , CandidateInfo :: Tag { name } => write ! (f , "tag {name:?}") , CandidateInfo :: Object { kind } => std :: fmt :: Display :: fmt (kind , f) , CandidateInfo :: Commit { date , title } => { write ! (f , "commit {} {title:?}" , gix_date :: parse_header (date) . unwrap_or_default () . format_or_unix (gix_date :: time :: format :: SHORT)) } } } }
    };
}

impl_876!()