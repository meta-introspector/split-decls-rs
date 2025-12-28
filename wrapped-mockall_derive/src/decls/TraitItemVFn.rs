macro_rules! TraitItemVFn {
    () => {
        # [doc = " Like a TraitItemFn, but with a visibility"] struct TraitItemVFn { pub vis : Visibility , pub tif : TraitItemFn }
    };
}

TraitItemVFn!()