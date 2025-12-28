macro_rules! copy_and_push {
    () => {
        # [doc = " This function copies the slice into a vec and appends an element to its end."] # [doc = " The vec is allocated with reserve_exact."] pub (crate) fn copy_and_push < T : Copy > (data : & [T] , to_push : T) -> alloc :: vec :: Vec < T > { let mut copy = alloc :: vec :: Vec :: new () ; copy . reserve_exact (data . len () + 1) ; copy . extend_from_slice (data) ; copy . push (to_push) ; copy }
    };
}

copy_and_push!();