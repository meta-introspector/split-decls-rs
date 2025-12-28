macro_rules! deps {
    () => {
        MutexGuard!();
        MappedMutexGuard!();
    };
}

macro_rules! impl_1307 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > MutexGuard < 'a , T > { # [doc = " Returns a locked view over a portion of the locked data."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::lock::{Mutex, MutexGuard};"] # [doc = ""] # [doc = " let data = Mutex::new(Some(\"value\".to_string()));"] # [doc = " {"] # [doc = "     let locked_str = MutexGuard::map(data.lock().await, |opt| opt.as_mut().unwrap());"] # [doc = "     assert_eq!(&*locked_str, \"value\");"] # [doc = " }"] # [doc = " # });"] # [doc = " ```"] # [inline] pub fn map < U : ? Sized , F > (this : Self , f : F) -> MappedMutexGuard < 'a , T , U > where F : FnOnce (& mut T) -> & mut U , { let mutex = this . mutex ; let value = f (unsafe { & mut * this . mutex . value . get () }) ; mem :: forget (this) ; MappedMutexGuard { mutex , value , _marker : PhantomData } } }
    };
}

impl_1307!()