macro_rules! NOOP_WAKER_VTABLE {
    () => {
        const NOOP_WAKER_VTABLE : RawWakerVTable = RawWakerVTable :: new (noop_clone , noop , noop , noop) ;
    };
}

NOOP_WAKER_VTABLE!()