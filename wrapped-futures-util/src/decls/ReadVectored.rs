macro_rules! ReadVectored {
    () => {
        # [doc = " Future for the [`read_vectored`](super::AsyncReadExt::read_vectored) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ReadVectored < 'a , 'b , R : ? Sized > { reader : & 'a mut R , bufs : & 'a mut [IoSliceMut < 'b >] , }
    };
}

ReadVectored!()