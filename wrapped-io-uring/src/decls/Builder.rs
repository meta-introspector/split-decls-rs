macro_rules! deps {
    () => {
        Entry!();
        IoUring!();
        EntryMarker!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " IoUring build params"] # [derive (Clone , Default)] pub struct Builder < S = squeue :: Entry , C = cqueue :: Entry > where S : squeue :: EntryMarker , C : cqueue :: EntryMarker , { dontfork : bool , params : sys :: io_uring_params , phantom : PhantomData < (S , C) > , }
    };
}

Builder!()