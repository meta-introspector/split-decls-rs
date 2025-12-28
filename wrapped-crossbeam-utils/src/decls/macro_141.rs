macro_rules! deps {
    () => {
        Registration!();
    };
}

macro_rules! macro_141 {
    () => {
        deps!();
        std :: thread_local ! { static REGISTRATION : Registration = { let thread_id = thread :: current () . id () ; let mut indices = thread_indices () . lock () . unwrap () ; let index = match indices . free_list . pop () { Some (i) => i , None => { let i = indices . next_index ; indices . next_index += 1 ; i } } ; indices . mapping . insert (thread_id , index) ; Registration { index , thread_id , } } ; }
    };
}

macro_141!()