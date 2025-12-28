macro_rules! DirOptions {
    () => {
        # [derive (Clone , Default)] pub struct DirOptions { # [doc = " Sets levels reading. Set value 0 for read all directory folder. By default 0."] pub depth : u64 , }
    };
}

DirOptions!()