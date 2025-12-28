macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! checkpoint {
    () => {
        deps!();
        # [cfg (not (feature = "checkpoint"))] mod checkpoint { use std :: path :: Path ; pub (crate) fn load_execution_path (_fs_path : & Path) -> crate :: rt :: Path { panic ! ("not compiled with `checkpoint` feature") } pub (crate) fn store_execution_path (_path : & crate :: rt :: Path , _fs_path : & Path) { panic ! ("not compiled with `checkpoint` feature") } }
    };
}

checkpoint!()