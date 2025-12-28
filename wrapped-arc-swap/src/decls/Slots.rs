macro_rules! deps {
    () => {
        Handover!();
        Debt!();
    };
}

macro_rules! Slots {
    () => {
        deps!();
        # [doc = " The slots for the helping strategy."] pub (super) struct Slots { # [doc = " The control structure of the slot."] # [doc = ""] # [doc = " Different threads signal what stage they are in in there. It can contain:"] # [doc = ""] # [doc = " * `IDLE` (nothing is happening, and there may or may not be an active debt)."] # [doc = " * a generation, tagged with GEN_TAG. The reader is trying to acquire a slot right now and a"] # [doc = "   writer might try to help out."] # [doc = " * A replacement pointer, tagged with REPLACEMENT_TAG. This pointer points to an Handover,"] # [doc = "   containing an already protected value, provided by the writer for the benefit of the"] # [doc = "   reader. The reader should abort its own debt and use this instead. This indirection"] # [doc = "   (storing pointer to the envelope with the actual pointer) is to make sure there's a space"] # [doc = "   for the tag ‒ there is no guarantee the real pointer is aligned to at least 4 bytes, we"] # [doc = "   can however force that for the Handover type."] control : AtomicUsize , # [doc = " A possibly active debt."] slot : Debt , # [doc = " If there's a generation in control, this signifies what address the reader is trying to"] # [doc = " load from."] active_addr : AtomicUsize , # [doc = " A place where a writer can put a replacement value."] # [doc = ""] # [doc = " Note that this is simply an allocation, and every participating slot contributes one, but"] # [doc = " they may be passed around through the lifetime of the program. It is not accessed directly,"] # [doc = " but through the space_offer thing."] # [doc = ""] handover : Handover , # [doc = " A pointer to a handover envelope this node currently owns."] # [doc = ""] # [doc = " A writer makes a switch of its and readers handover when successfully storing a replacement"] # [doc = " in the control."] space_offer : AtomicPtr < Handover > , }
    };
}

Slots!();