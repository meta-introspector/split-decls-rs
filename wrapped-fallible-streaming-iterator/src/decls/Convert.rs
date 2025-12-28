macro_rules! Convert {
    () => {
        # [doc = " An iterator which wraps a normal `Iterator`."] pub struct Convert < 'a , I , T : 'a > { it : I , item : Option < & 'a T > , }
    };
}

Convert!();