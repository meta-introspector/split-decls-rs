macro_rules! deps {
    () => {
        Reader!();
        Vendor!();
    };
}

macro_rules! EhFrame {
    () => {
        deps!();
        # [doc = " `EhFrame` contains the frame unwinding information needed during exception"] # [doc = " handling found in the `.eh_frame` section."] # [doc = ""] # [doc = " Most interesting methods are defined in the"] # [doc = " [`UnwindSection`](trait.UnwindSection.html) trait."] # [doc = ""] # [doc = " See"] # [doc = " [`DebugFrame`](./struct.DebugFrame.html#differences-between-debug_frame-and-eh_frame)"] # [doc = " for some discussion on the differences between `.debug_frame` and"] # [doc = " `.eh_frame`."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct EhFrame < R : Reader > { section : R , address_size : u8 , vendor : Vendor , }
    };
}

EhFrame!()