macro_rules! FilterElements {
    () => {
        # [doc = " An iterator that filters graph elements."] # [doc = ""] # [doc = " See [`.filter_elements()`][1] for more information."] # [doc = ""] # [doc = " [1]: trait.ElementIterator.html#method.filter_elements"] # [derive (Debug , Clone)] pub struct FilterElements < I , F > { iter : I , node_index : usize , map : Vec < usize > , f : F , }
    };
}

FilterElements!();