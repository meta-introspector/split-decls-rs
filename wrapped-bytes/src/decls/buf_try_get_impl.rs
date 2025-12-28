macro_rules! deps {
    () => {
        TryGetError!();
    };
}

macro_rules! buf_try_get_impl {
    () => {
        deps!();
        macro_rules ! buf_try_get_impl { ($ this : ident , $ typ : tt ::$ conv : tt) => { { const SIZE : usize = core :: mem :: size_of ::<$ typ > () ; if $ this . remaining () < SIZE { return Err (TryGetError { requested : SIZE , available : $ this . remaining () , }) ; } let ret = $ this . chunk () . get (.. SIZE) . map (| src | unsafe { $ typ ::$ conv (* (src as * const _ as * const [_ ; SIZE])) }) ; if let Some (ret) = ret { $ this . advance (SIZE) ; return Ok (ret) ; } else { let mut buf = [0 ; SIZE] ; $ this . copy_to_slice (& mut buf) ; return Ok ($ typ ::$ conv (buf)) ; } } } ; (le => $ this : ident , $ typ : tt , $ len_to_read : expr) => { { const SIZE : usize = core :: mem :: size_of ::<$ typ > () ; let mut buf = [0 ; SIZE] ; let subslice = match buf . get_mut (..$ len_to_read) { Some (subslice) => subslice , None => panic_does_not_fit (SIZE , $ len_to_read) , } ; $ this . try_copy_to_slice (subslice) ?; return Ok ($ typ :: from_le_bytes (buf)) ; } } ; (be => $ this : ident , $ typ : tt , $ len_to_read : expr) => { { const SIZE : usize = core :: mem :: size_of ::<$ typ > () ; let slice_at = match SIZE . checked_sub ($ len_to_read) { Some (slice_at) => slice_at , None => panic_does_not_fit (SIZE , $ len_to_read) , } ; let mut buf = [0 ; SIZE] ; $ this . try_copy_to_slice (& mut buf [slice_at ..]) ?; return Ok ($ typ :: from_be_bytes (buf)) ; } } ; }
    };
}

buf_try_get_impl!();