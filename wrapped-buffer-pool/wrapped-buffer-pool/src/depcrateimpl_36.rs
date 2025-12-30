// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < const S : usize , T : Default + Reuse > Pool < S , T > { # [doc = " Get a value from the pool, or create a new default value if the"] # [doc = " assigned shard is currently empty."] pub fn get (& 'static self) -> Pooled < T > { let shard = self . next_shard . fetch_add (1 , Ordering :: Relaxed) % S ; let shard = & self . queues [shard] ; let inner = match shard . queue . pop () { Some (el) => { shard . elem_cnt . fetch_sub (1 , Ordering :: Relaxed) ; el } , None => Default :: default () , } ; Pooled { inner , pool : shard } } # [doc = " Create a new default value assigned for a pool, if it is ends up"] # [doc = " being expanded and eligible for reuse it will return to the pool,"] # [doc = " otherwise it will end up being dropped."] pub fn get_empty (& 'static self) -> Pooled < T > { let shard = self . next_shard . load (Ordering :: Relaxed) % S ; let shard = & self . queues [shard] ; Pooled { inner : Default :: default () , pool : shard , } } # [doc = " Get a value from the pool and apply the provided transformation on"] # [doc = " it before returning."] pub fn get_with (& 'static self , f : impl Fn (& mut T)) -> Pooled < T > { let mut pooled = self . get () ; f (& mut pooled) ; pooled } pub fn from_owned (& 'static self , inner : T) -> Pooled < T > { let shard = self . next_shard . fetch_add (1 , Ordering :: Relaxed) % S ; let shard = & self . queues [shard] ; Pooled { inner , pool : shard } } }
};
}
