macro_rules! deps {
    () => {
        RefCnt!();
    };
}

macro_rules! Strategy {
    () => {
        deps!();
        # [doc = " A strategy for protecting the reference counted pointer `T`."] # [doc = ""] # [doc = " This chooses the algorithm for how the reference counts are protected. Note that the user of"] # [doc = " the crate can't implement the trait and can't access any method; this is hopefully temporary"] # [doc = " measure to make sure the interface is not part of the stability guarantees of the crate. Once"] # [doc = " enough experience is gained with implementing various strategies, it will be un-sealed and"] # [doc = " users will be able to provide their own implementation."] # [doc = ""] # [doc = " For now, the trait works only as a bound to talk about the types that represent strategies."] pub trait Strategy < T : RefCnt > : sealed :: InnerStrategy < T > { }
    };
}

Strategy!();