macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        # [doc = " Convert from `Either` to `Result` with `Right => Ok` and `Left => Err`."] impl < L , R > From < Either < L , R > > for Result < R , L > { fn from (val : Either < L , R >) -> Self { match val { Left (l) => Err (l) , Right (r) => Ok (r) , } } }
    };
}

impl_41!()