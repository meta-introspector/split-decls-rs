macro_rules! deps {
    () => {
        RegisterRule!();
        ReaderOffset!();
        UnwindContext!();
        UnwindTableRow!();
        ArrayLike!();
        StoreOnHeap!();
        Register!();
    };
}

macro_rules! UnwindContextStorage {
    () => {
        deps!();
        # [doc = " Specification of what storage should be used for [`UnwindContext`]."] # [doc = ""] # [cfg_attr (feature = "read" , doc = "
Normally you would only need to use [`StoreOnHeap`], which places the stack
on the heap using [`Box`]. This is the default storage type parameter for [`UnwindContext`].

You may want to supply your own storage type for one of the following reasons:

  1. In rare cases you may run into failed unwinds due to the fixed stack size
     used by [`StoreOnHeap`], so you may want to try a larger `Box`. If denial
     of service is not a concern, then you could also try a `Vec`-based stack which
     can grow as needed.
  2. You may want to avoid heap allocations entirely. You can use a fixed-size
     stack with in-line arrays, which will place the entire storage in-line into
     [`UnwindContext`].
")] # [doc = ""] # [doc = " Here's an implementation which uses a fixed-size stack and allocates everything in-line,"] # [doc = " which will cause `UnwindContext` to be large:"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use gimli::*;"] # [doc = " #"] # [doc = " # fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)"] # [doc = " #            -> gimli::Result<()> {"] # [doc = " # let eh_frame: gimli::EhFrame<_> = unreachable!();"] # [doc = " # let bases = unimplemented!();"] # [doc = " #"] # [doc = " struct StoreOnStack;"] # [doc = ""] # [doc = " impl<T: ReaderOffset> UnwindContextStorage<T> for StoreOnStack {"] # [doc = "     type Rules = [(Register, RegisterRule<T>); 192];"] # [doc = "     type Stack = [UnwindTableRow<T, Self>; 4];"] # [doc = " }"] # [doc = ""] # [doc = " let mut ctx = UnwindContext::<_, StoreOnStack>::new_in();"] # [doc = ""] # [doc = " // Initialize the context by evaluating the CIE's initial instruction program,"] # [doc = " // and generate the unwind table."] # [doc = " let mut table = some_fde.rows(&eh_frame, &bases, &mut ctx)?;"] # [doc = " while let Some(row) = table.next_row()? {"] # [doc = "     // Do stuff with each row..."] # [doc = " #   let _ = row;"] # [doc = " }"] # [doc = " # unreachable!()"] # [doc = " # }"] # [doc = " ```"] pub trait UnwindContextStorage < T : ReaderOffset > : Sized { # [doc = " The storage used for register rules in a unwind table row."] # [doc = ""] # [doc = " Note that this is nested within the stack."] type Rules : ArrayLike < Item = (Register , RegisterRule < T >) > ; # [doc = " The storage used for unwind table row stack."] type Stack : ArrayLike < Item = UnwindTableRow < T , Self > > ; }
    };
}

UnwindContextStorage!()