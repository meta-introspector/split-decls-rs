macro_rules! deps {
    () => {
        Inner!();
        Entry!();
        EntryMarker!();
        Parameters!();
        MemoryMap!();
    };
}

macro_rules! IoUring {
    () => {
        deps!();
        # [doc = " IoUring instance"] # [doc = ""] # [doc = " - `S`: The ring's submission queue entry (SQE) type, either [`squeue::Entry`] or"] # [doc = "   [`squeue::Entry128`];"] # [doc = " - `C`: The ring's completion queue entry (CQE) type, either [`cqueue::Entry`] or"] # [doc = "   [`cqueue::Entry32`]."] pub struct IoUring < S = squeue :: Entry , C = cqueue :: Entry > where S : squeue :: EntryMarker , C : cqueue :: EntryMarker , { sq : squeue :: Inner < S > , cq : cqueue :: Inner < C > , fd : OwnedFd , params : Parameters , memory : ManuallyDrop < MemoryMap > , }
    };
}

IoUring!();