macro_rules! subspan {
    () => {
        # [doc = " Returns a subspan of the given span."] # [doc = ""] # [doc = " TODO: the implementation is really... wtf! But i didn't find a better way to do it."] fn subspan < R : RangeBounds < usize > > (span : Span , range : R) -> Option < Span > { let mut lit = proc_macro2 :: Literal :: i8_suffixed (0) ; lit . set_span (span) ; lit . subspan (range) }
    };
}

subspan!();