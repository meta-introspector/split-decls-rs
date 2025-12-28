macro_rules! deps {
    () => {
        Measurement!();
        Function!();
        Bencher!();
        Duration!();
        Routine!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < M : Measurement , F , T > Routine < M , T > for Function < M , F , T > where F : FnMut (& mut Bencher < '_ , M > , & T) , T : ? Sized , { fn bench (& mut self , m : & M , iters : & [u64] , parameter : & T) -> Vec < f64 > { let f = & mut self . f ; let mut b = Bencher { iterated : false , iters : 0 , value : m . zero () , measurement : m , elapsed_time : Duration :: from_millis (0) , } ; iters . iter () . map (| iters | { b . iters = * iters ; (* f) (& mut b , black_box (parameter)) ; b . assert_iterated () ; m . to_f64 (& b . value) }) . collect () } fn warm_up (& mut self , m : & M , how_long : Duration , parameter : & T) -> (u64 , u64) { let f = & mut self . f ; let mut b = Bencher { iterated : false , iters : 1 , value : m . zero () , measurement : m , elapsed_time : Duration :: from_millis (0) , } ; let mut total_iters = 0 ; let mut elapsed_time = Duration :: from_millis (0) ; loop { (* f) (& mut b , black_box (parameter)) ; b . assert_iterated () ; total_iters += b . iters ; elapsed_time += b . elapsed_time ; if elapsed_time > how_long { return (elapsed_time . as_nanos () as u64 , total_iters) ; } b . iters = b . iters . wrapping_mul (2) ; } } }
    };
}

impl_290!()