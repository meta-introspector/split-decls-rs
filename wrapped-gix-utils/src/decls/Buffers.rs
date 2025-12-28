macro_rules! Buffers {
    () => {
        # [doc = " A utility to do buffer-swapping with."] # [doc = ""] # [doc = " Use `src` to read from and `dest` to write to, and after actually changing data, call [Buffers::swap()]."] # [doc = " To be able to repeat the process, this time using what was `dest` as `src`, freeing up `dest` for writing once more."] # [doc = ""] # [doc = " Note that after each [`Buffers::swap()`], `src` is the most recent version of the data, just like before each swap."] # [derive (Default , Clone)] pub struct Buffers { # [doc = " The source data, as basis for processing."] pub src : Vec < u8 > , # [doc = " The data produced after processing `src`."] pub dest : Vec < u8 > , }
    };
}

Buffers!()