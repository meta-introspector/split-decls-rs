macro_rules! deps {
    () => {
        FluentValue!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use crate :: types :: FluentValue ; # [test] fn value_from_copy_ref () { let x = 1i16 ; let y = & x ; let z : FluentValue = y . into () ; assert_eq ! (z , FluentValue :: try_number ("1")) ; } }
    };
}

tests!();