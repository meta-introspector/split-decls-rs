macro_rules! deps {
    () => {
        LookupContinuation!();
        SplitDwarfLoad!();
    };
}

macro_rules! LookupResult {
    () => {
        deps!();
        # [doc = " Operations that consult debug information may require additional files"] # [doc = " to be loaded if split DWARF is being used. This enum returns the result"] # [doc = " of the operation in the `Output` variant, or information about the split"] # [doc = " DWARF that is required and a continuation to invoke once it is available"] # [doc = " in the `Load` variant."] # [doc = ""] # [doc = " This enum is intended to be used in a loop like so:"] # [doc = " ```no_run"] # [doc = "   # use addr2line::*;"] # [doc = "   # use std::sync::Arc;"] # [doc = "   # let ctx: Context<gimli::EndianSlice<gimli::RunTimeEndian>> = todo!();"] # [doc = "   # let do_split_dwarf_load = |load: SplitDwarfLoad<gimli::EndianSlice<gimli::RunTimeEndian>>| -> Option<Arc<gimli::Dwarf<gimli::EndianSlice<gimli::RunTimeEndian>>>> { None };"] # [doc = "   const ADDRESS: u64 = 0xdeadbeef;"] # [doc = "   let mut r = ctx.find_frames(ADDRESS);"] # [doc = "   let result = loop {"] # [doc = "     match r {"] # [doc = "       LookupResult::Output(result) => break result,"] # [doc = "       LookupResult::Load { load, continuation } => {"] # [doc = "         let dwo = do_split_dwarf_load(load);"] # [doc = "         r = continuation.resume(dwo);"] # [doc = "       }"] # [doc = "     }"] # [doc = "   };"] # [doc = " ```"] pub enum LookupResult < L : LookupContinuation > { # [doc = " The lookup requires split DWARF data to be loaded."] Load { # [doc = " The information needed to find the split DWARF data."] load : SplitDwarfLoad < < L as LookupContinuation > :: Buf > , # [doc = " The continuation to resume with the loaded split DWARF data."] continuation : L , } , # [doc = " The lookup has completed and produced an output."] Output (< L as LookupContinuation > :: Output) , }
    };
}

LookupResult!();