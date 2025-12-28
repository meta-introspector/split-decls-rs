macro_rules! deps {
    () => {
        ErrorExtensionValues!();
        Result!();
    };
}

macro_rules! ResultExt {
    () => {
        deps!();
        # [doc = " Extend a `Result`'s error value with"] # [doc = " [`ErrorExtensions`](trait.ErrorExtensions.html)."] pub trait ResultExt < T , E > : Sized { # [doc = " Extend the error value of the result with the callback."] fn extend_err < C > (self , cb : C) -> Result < T > where C : FnOnce (& E , & mut ErrorExtensionValues) ; # [doc = " Extend the result to a `Result`."] fn extend (self) -> Result < T > ; }
    };
}

ResultExt!()