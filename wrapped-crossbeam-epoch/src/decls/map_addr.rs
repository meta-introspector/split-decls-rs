macro_rules! map_addr {
    () => {
        # [inline] fn map_addr < T > (ptr : * mut T , f : impl FnOnce (usize) -> usize) -> * mut T { let new_addr = f (ptr as usize) ; ptr . cast :: < u8 > () . wrapping_add (new_addr . wrapping_sub (ptr as usize)) . cast :: < T > () }
    };
}

map_addr!()