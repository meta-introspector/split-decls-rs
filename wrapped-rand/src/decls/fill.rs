macro_rules! fill {
    () => {
        # [doc = " Fill any type implementing [`Fill`] with random data"] # [doc = ""] # [doc = " This function is shorthand for"] # [doc = " <code>[rng()].[fill](Rng::fill)(<var>dest</var>)</code>."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let mut arr = [0i8; 20];"] # [doc = " rand::fill(&mut arr[..]);"] # [doc = " ```"] # [doc = ""] # [doc = " Note that you can instead use [`random()`] to generate an array of random"] # [doc = " data, though this is slower for small elements (smaller than the RNG word"] # [doc = " size)."] # [cfg (feature = "thread_rng")] # [inline] # [track_caller] pub fn fill < T : Fill > (dest : & mut [T]) { Fill :: fill_slice (dest , & mut rng ()) }
    };
}

fill!()