macro_rules! deps {
    () => {
        FluentValue!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'source , T > From < Option < T > > for FluentValue < 'source > where T : Into < FluentValue < 'source > > , { fn from (v : Option < T >) -> Self { match v { Some (v) => v . into () , None => FluentValue :: None , } } }
    };
}

impl_104!();