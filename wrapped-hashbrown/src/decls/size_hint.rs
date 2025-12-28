macro_rules! size_hint {
    () => {
        mod size_hint { use core :: cmp ; # [doc = " This presumably exists to prevent denial of service attacks."] # [doc = ""] # [doc = " Original discussion: https://github.com/serde-rs/serde/issues/1114."] # [cfg_attr (feature = "inline-more" , inline)] pub (super) fn cautious (hint : Option < usize >) -> usize { cmp :: min (hint . unwrap_or (0) , 4096) } }
    };
}

size_hint!();