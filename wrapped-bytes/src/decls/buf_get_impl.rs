macro_rules! buf_get_impl {
    () => {
        macro_rules ! buf_get_impl { ($ this : ident , $ typ : tt ::$ conv : tt) => { { return (|| buf_try_get_impl ! ($ this , $ typ ::$ conv)) () . unwrap_or_else (| error | panic_advance (& error)) ; } } ; (le => $ this : ident , $ typ : tt , $ len_to_read : expr) => { { return (|| buf_try_get_impl ! (le => $ this , $ typ , $ len_to_read)) () . unwrap_or_else (| error | panic_advance (& error)) ; } } ; (be => $ this : ident , $ typ : tt , $ len_to_read : expr) => { { return (|| buf_try_get_impl ! (be => $ this , $ typ , $ len_to_read)) () . unwrap_or_else (| error | panic_advance (& error)) ; } } ; }
    };
}

buf_get_impl!()