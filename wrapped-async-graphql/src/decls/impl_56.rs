macro_rules! deps {
    () => {
        ResultExt!();
        Result!();
        ErrorExtensions!();
        ErrorExtensionValues!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T , E > ResultExt < T , E > for std :: result :: Result < T , E > where E : ErrorExtensions + Send + Sync + 'static , { fn extend_err < C > (self , cb : C) -> Result < T > where C : FnOnce (& E , & mut ErrorExtensionValues) , { match self { Err (err) => Err (err . extend_with (| e , ee | cb (e , ee))) , Ok (value) => Ok (value) , } } fn extend (self) -> Result < T > { match self { Err (err) => Err (err . extend ()) , Ok (value) => Ok (value) , } } }
    };
}

impl_56!();