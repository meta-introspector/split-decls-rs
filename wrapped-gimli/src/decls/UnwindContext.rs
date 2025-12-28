macro_rules! deps {
    () => {
        Register!();
        UnwindTable!();
        UnwindContextStorage!();
        StoreOnHeap!();
        ReaderOffset!();
        ArrayVec!();
        RegisterRule!();
    };
}

macro_rules! UnwindContext {
    () => {
        deps!();
        # [doc = " Common context needed when evaluating the call frame unwinding information."] # [doc = ""] # [doc = " By default, this structure is small and allocates its internal storage"] # [doc = " on the heap using [`Box`] during [`UnwindContext::new`]."] # [doc = ""] # [doc = " This can be overridden by providing a custom [`UnwindContextStorage`] type parameter."] # [doc = " When using a custom storage with in-line arrays, the [`UnwindContext`] type itself"] # [doc = " will be big, so in that case it's recommended to place [`UnwindContext`] on the"] # [doc = " heap, e.g. using `Box::new(UnwindContext::<R, MyCustomStorage>::new_in())`."] # [doc = ""] # [doc = " To avoid re-allocating the context multiple times when evaluating multiple"] # [doc = " CFI programs, the same [`UnwindContext`] can be reused for multiple unwinds."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{UnwindContext, UnwindTable};"] # [doc = ""] # [doc = " # fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)"] # [doc = " #            -> gimli::Result<()> {"] # [doc = " # let eh_frame: gimli::EhFrame<_> = unreachable!();"] # [doc = " # let bases = unimplemented!();"] # [doc = " // An uninitialized context."] # [doc = " let mut ctx = UnwindContext::new();"] # [doc = ""] # [doc = " // Initialize the context by evaluating the CIE's initial instruction program,"] # [doc = " // and generate the unwind table."] # [doc = " let mut table = some_fde.rows(&eh_frame, &bases, &mut ctx)?;"] # [doc = " while let Some(row) = table.next_row()? {"] # [doc = "     // Do stuff with each row..."] # [doc = " #   let _ = row;"] # [doc = " }"] # [doc = " # unreachable!()"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , PartialEq , Eq)] pub struct UnwindContext < T , S = StoreOnHeap > where T : ReaderOffset , S : UnwindContextStorage < T > , { stack : ArrayVec < S :: Stack > , initial_rule : Option < (Register , RegisterRule < T >) > , is_initialized : bool , }
    };
}

UnwindContext!();