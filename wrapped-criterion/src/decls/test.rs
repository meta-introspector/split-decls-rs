macro_rules! test {
    () => {
        # [cfg (test)] mod test { test ! (f32) ; test ! (f64) ; }
    };
}

test!()