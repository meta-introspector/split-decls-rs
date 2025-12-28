macro_rules! checked_add_opt {
    () => {
        # [inline (always)] fn checked_add_opt (one : Option < usize > , other : Option < usize >) -> Option < usize > { if let Some (n) = one { checked_add (n , other) } else { None } }
    };
}

checked_add_opt!();