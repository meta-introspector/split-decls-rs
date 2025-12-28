macro_rules! WriterThread {
    () => {
        # [doc = " A helper to manage writing to stdin on a separate thread to avoid deadlock."] struct WriterThread { handle : Option < std :: thread :: JoinHandle < std :: io :: Result < () > > > , }
    };
}

WriterThread!();