macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! Lazy {
    () => {
        deps!();
        # [doc = " A lazily initialized value that implements `Deref` for `T`."] # [doc = ""] # [doc = " A `Lazy` takes an initialization function and permits callers from any"] # [doc = " thread to access the result of that initialization function in a safe"] # [doc = " manner. In effect, this permits one-time initialization of global resources"] # [doc = " in a (possibly) multi-threaded program."] # [doc = ""] # [doc = " This type and its functionality are available even when neither the `alloc`"] # [doc = " nor the `std` features are enabled. In exchange, a `Lazy` does **not**"] # [doc = " guarantee that the given `create` function is called at most once. It"] # [doc = " might be called multiple times. Moreover, a call to `Lazy::get` (either"] # [doc = " explicitly or implicitly via `Lazy`'s `Deref` impl) may block until a `T`"] # [doc = " is available."] # [doc = ""] # [doc = " This is very similar to `lazy_static` or `once_cell`, except it doesn't"] # [doc = " guarantee that the initialization function will be run once and it works"] # [doc = " in no-alloc no-std environments. With that said, if you need stronger"] # [doc = " guarantees or a more flexible API, then it is recommended to use either"] # [doc = " `lazy_static` or `once_cell`."] # [doc = ""] # [doc = " # Warning: may use a spin lock"] # [doc = ""] # [doc = " When this crate is compiled _without_ the `alloc` feature, then this type"] # [doc = " may used a spin lock internally. This can have subtle effects that may"] # [doc = " be undesirable. See [Spinlocks Considered Harmful][spinharm] for a more"] # [doc = " thorough treatment of this topic."] # [doc = ""] # [doc = " [spinharm]: https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This type is useful for creating regexes once, and then using them from"] # [doc = " multiple threads simultaneously without worrying about synchronization."] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{dfa::regex::Regex, util::lazy::Lazy, Match};"] # [doc = ""] # [doc = " static RE: Lazy<Regex> = Lazy::new(|| Regex::new(\"foo[0-9]+bar\").unwrap());"] # [doc = ""] # [doc = " let expected = Some(Match::must(0, 3..14));"] # [doc = " assert_eq!(expected, RE.find(b\"zzzfoo12345barzzz\"));"] # [doc = " ```"] pub struct Lazy < T , F = fn () -> T > (lazy :: Lazy < T , F >) ;
    };
}

Lazy!()