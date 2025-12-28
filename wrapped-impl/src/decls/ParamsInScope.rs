macro_rules! ParamsInScope {
    () => {
        pub struct ParamsInScope < 'a > { names : Set < & 'a Ident > , }
    };
}

ParamsInScope!()