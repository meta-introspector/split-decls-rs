macro_rules! deps {
    () => {
        RefLogMessage!();
    };
}

macro_rules! impl_949 {
    () => {
        deps!();
        impl RefLogMessage { pub (crate) fn compose (& self , context : & str) -> BString { match self { RefLogMessage :: Prefixed { action } => format ! ("{action}: {context}") . into () , RefLogMessage :: Override { message } => message . to_owned () , } } }
    };
}

impl_949!();