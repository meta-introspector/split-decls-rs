macro_rules! Scope {
    () => {
        # [doc = " passed in scope type"] # [doc = " it not use the context to pass data, but keep it's own data ref"] # [doc = " this struct provide both compile type info and runtime data"] pub struct Scope < 'scope , 'a , A , T > { para : & 'a mut Option < A > , ret : & 'a mut Option < T > , scope : PhantomData < & 'scope mut & 'scope () > , }
    };
}

Scope!();