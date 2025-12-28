macro_rules! deps {
    () => {
        Vendor!();
        Reader!();
    };
}

macro_rules! DebugFrame {
    () => {
        deps!();
        # [doc = " `DebugFrame` contains the `.debug_frame` section's frame unwinding"] # [doc = " information required to unwind to and recover registers from older frames on"] # [doc = " the stack. For example, this is useful for a debugger that wants to print"] # [doc = " locals in a backtrace."] # [doc = ""] # [doc = " Most interesting methods are defined in the"] # [doc = " [`UnwindSection`](trait.UnwindSection.html) trait."] # [doc = ""] # [doc = " ### Differences between `.debug_frame` and `.eh_frame`"] # [doc = ""] # [doc = " While the `.debug_frame` section's information has a lot of overlap with the"] # [doc = " `.eh_frame` section's information, the `.eh_frame` information tends to only"] # [doc = " encode the subset of information needed for exception handling. Often, only"] # [doc = " one of `.eh_frame` or `.debug_frame` will be present in an object file."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct DebugFrame < R : Reader > { section : R , address_size : u8 , vendor : Vendor , }
    };
}

DebugFrame!();