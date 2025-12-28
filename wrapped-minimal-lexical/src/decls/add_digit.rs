macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! add_digit {
    () => {
        deps!();
        # [doc = " Add a digit to the temporary value."] macro_rules ! add_digit { ($ c : ident , $ value : ident , $ counter : ident , $ count : ident) => { { let digit = $ c - b'0' ; $ value *= 10 as Limb ; $ value += digit as Limb ; $ counter += 1 ; $ count += 1 ; } } ; }
    };
}

add_digit!()