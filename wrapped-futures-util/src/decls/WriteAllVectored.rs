macro_rules! WriteAllVectored {
    () => {
        # [doc = " Future for the"] # [doc = " [`write_all_vectored`](super::AsyncWriteExt::write_all_vectored) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct WriteAllVectored < 'a , 'b , W : ? Sized + Unpin > { writer : & 'a mut W , bufs : & 'a mut [IoSlice < 'b >] , }
    };
}

WriteAllVectored!()