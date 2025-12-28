macro_rules! deps {
    () => {
        DisambiguatedDefPathData!();
        DefPathData!();
        DefPathDataName!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl DisambiguatedDefPathData { pub fn as_sym (& self , verbose : bool) -> Symbol { match self . data . name () { DefPathDataName :: Named (name) => { if verbose && self . disambiguator != 0 { Symbol :: intern (& format ! ("{}#{}" , name , self . disambiguator)) } else { name } } DefPathDataName :: Anon { namespace } => { if let DefPathData :: AnonAssocTy (method) = self . data { Symbol :: intern (& format ! ("{}::{{{}#{}}}" , method , namespace , self . disambiguator)) } else { Symbol :: intern (& format ! ("{{{}#{}}}" , namespace , self . disambiguator)) } } } } }
    };
}

impl_91!()