macro_rules! deps {
    () => {
        Url!();
    };
}

macro_rules! testing {
    () => {
        deps!();
        # [doc = " This module contains extensions to the [Url] struct which are only intended to be used"] # [doc = " for testing code. Do not use this module in production! For all intents and purposes, the APIs of"] # [doc = " all functions and types exposed by this module are considered unstable and are allowed to break"] # [doc = " even in patch releases!"] # [doc (hidden)] pub mod testing { use bstr :: BString ; use crate :: { Scheme , Url } ; # [doc = " Additional functions for [Url] which are only intended to be used for tests."] pub trait TestUrlExtension { # [doc = " Create a new instance from the given parts without validating them."] # [doc = ""] # [doc = " This function is primarily intended for testing purposes. For production code please"] # [doc = " consider using [Url::from_parts] instead!"] fn from_parts_unchecked (scheme : Scheme , user : Option < String > , password : Option < String > , host : Option < String > , port : Option < u16 > , path : BString , serialize_alternative_form : bool ,) -> Url { Url { scheme , user , password , host , port , path , serialize_alternative_form , } } } impl TestUrlExtension for Url { } }
    };
}

testing!()