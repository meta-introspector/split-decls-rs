macro_rules! fuse {
    () => {
        # [doc = " Fuse a future such that `poll` will never again be called once it has"] # [doc = " completed. This method can be used to turn any `Future` into a"] # [doc = " `FusedFuture`."] # [doc = ""] # [doc = " Normally, once a future has returned `Poll::Ready` from `poll`,"] # [doc = " any further calls could exhibit bad behavior such as blocking"] # [doc = " forever, panicking, never returning, etc. If it is known that `poll`"] # [doc = " may be called too often then this method can be used to ensure that it"] # [doc = " has defined semantics."] # [doc = ""] # [doc = " If a `fuse`d future is `poll`ed after having returned `Poll::Ready`"] # [doc = " previously, it will return `Poll::Pending`, from `poll` again (and will"] # [doc = " continue to do so for all future calls to `poll`)."] # [doc = ""] # [doc = " This combinator will drop the underlying future as soon as it has been"] # [doc = " completed to ensure resources are reclaimed as soon as possible."] pub fn fuse < F > (future : F) -> Fuse < F > where F : Future + Sized , { Fuse :: new (future) }
    };
}

fuse!()