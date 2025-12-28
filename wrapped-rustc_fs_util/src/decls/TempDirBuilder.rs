macro_rules! TempDirBuilder {
    () => {
        pub struct TempDirBuilder < 'a , 'b > { builder : tempfile :: Builder < 'a , 'b > , }
    };
}

TempDirBuilder!()