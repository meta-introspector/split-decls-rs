macro_rules! and_all {
    () => {
        # [doc = " Take the sum of all of the given size hints."] # [doc = ""] # [doc = " If `hints` is empty, returns `(0, Some(0))`, aka the size of consuming"] # [doc = " nothing."] # [inline] pub fn and_all (hints : & [(usize , Option < usize >)]) -> (usize , Option < usize >) { hints . iter () . copied () . fold ((0 , Some (0)) , and) }
    };
}

and_all!();