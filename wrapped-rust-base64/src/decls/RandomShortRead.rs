macro_rules! RandomShortRead {
    () => {
        # [doc = " Limits how many bytes a reader will provide in each read call."] # [doc = " Useful for shaking out code that may work fine only with typical input sources that always fill"] # [doc = " the buffer."] struct RandomShortRead < 'a , 'b , R : io :: Read , N : rand :: Rng > { delegate : & 'b mut R , rng : & 'a mut N , }
    };
}

RandomShortRead!()