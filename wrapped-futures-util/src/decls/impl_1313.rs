macro_rules! deps {
    () => {
        MutexGuard!();
        MappedMutexGuard!();
        Mutex!();
    };
}

macro_rules! impl_1313 {
    () => {
        deps!();
        impl < 'a , T : ? Sized , U : ? Sized > MappedMutexGuard < 'a , T , U > { # [doc = " Returns a locked view over a portion of the locked data."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::lock::{MappedMutexGuard, Mutex, MutexGuard};"] # [doc = ""] # [doc = " let data = Mutex::new(Some(\"value\".to_string()));"] # [doc = " {"] # [doc = "     let locked_str = MutexGuard::map(data.lock().await, |opt| opt.as_mut().unwrap());"] # [doc = "     let locked_char = MappedMutexGuard::map(locked_str, |s| s.get_mut(0..1).unwrap());"] # [doc = "     assert_eq!(&*locked_char, \"v\");"] # [doc = " }"] # [doc = " # });"] # [doc = " ```"] # [inline] pub fn map < V : ? Sized , F > (this : Self , f : F) -> MappedMutexGuard < 'a , T , V > where F : FnOnce (& mut U) -> & mut V , { let mutex = this . mutex ; let value = f (unsafe { & mut * this . value }) ; mem :: forget (this) ; MappedMutexGuard { mutex , value , _marker : PhantomData } } }
    };
}

impl_1313!()