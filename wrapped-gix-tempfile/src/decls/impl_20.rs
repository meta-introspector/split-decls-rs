macro_rules! deps {
    () => {
        ContainingDirectory!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl ContainingDirectory { fn resolve (self , dir : & Path) -> std :: io :: Result < & Path > { match self { ContainingDirectory :: Exists => Ok (dir) , ContainingDirectory :: CreateAllRaceProof (retries) => crate :: create_dir :: all (dir , retries) , } } }
    };
}

impl_20!();