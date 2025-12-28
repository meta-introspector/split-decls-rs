macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Found {} {} the refspec mapping to be used: \n\t{}" , self . issues . len () , if self . issues . len () == 1 { "issue that prevents" } else { "issues that prevent" } , self . issues . iter () . map (ToString :: to_string) . collect ::< Vec < _ >> () . join ("\n\t")) } }
    };
}

impl_32!();