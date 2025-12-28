macro_rules! Iter {
    () => {
        # [doc = " An iterator over attribute assignments in a single line."] pub struct Iter < 'a > { attrs : bstr :: Fields < 'a > , }
    };
}

Iter!()