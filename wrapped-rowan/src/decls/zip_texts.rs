macro_rules! deps {
    () => {
        SyntaxToken!();
    };
}

macro_rules! zip_texts {
    () => {
        deps!();
        fn zip_texts < I : Iterator < Item = (SyntaxToken , TextRange) > > (xs : & mut I , ys : & mut I) -> Option < () > { let mut x = xs . next () ? ; let mut y = ys . next () ? ; loop { while x . 1 . is_empty () { x = xs . next () ? ; } while y . 1 . is_empty () { y = ys . next () ? ; } let x_text = & x . 0 . text () [x . 1] ; let y_text = & y . 0 . text () [y . 1] ; if ! (x_text . starts_with (y_text) || y_text . starts_with (x_text)) { return Some (()) ; } let advance = std :: cmp :: min (x . 1 . len () , y . 1 . len ()) ; x . 1 = TextRange :: new (x . 1 . start () + advance , x . 1 . end ()) ; y . 1 = TextRange :: new (y . 1 . start () + advance , y . 1 . end ()) ; } }
    };
}

zip_texts!();