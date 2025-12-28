macro_rules! deps {
    () => {
        AsyncTraitMode!();
    };
}

macro_rules! async_mode {
    () => {
        deps!();
        fn async_mode (arg : & str) -> Result < AsyncTraitMode > { match arg { "" | "Send" => Ok (AsyncTraitMode :: Send) , "?Send" => Ok (AsyncTraitMode :: NotSend) , "AFIT" => Ok (AsyncTraitMode :: Off) , _ => Err (syn :: Error :: new (Span :: call_site () , "Only accepts `Send`, `?Send` or `AFIT` (native async function in trait)" ,)) , } }
    };
}

async_mode!()