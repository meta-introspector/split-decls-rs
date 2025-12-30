// Generated macro for FairRateLimiter (struct)
macro_rules! DepcrateFairRateLimiter {
() => {
// Module: crate
// Provides: {"FairRateLimiter"}
// Dependencies: {}
# [doc = " A fair rate-limiter."] # [doc = " - Probabilistically rejects requests."] # [doc = " - When not overloaded, allows any source to freely consume throughput."] # [doc = " - Gradually increases fairness as load approaches overload."] # [doc = " - Onset of overload does not trigger a sudden total outage for any group of users."] # [doc = " - In overload, it tries to give every source the same throughput."] # [doc = " - A limited source of overload will get throttled and leave other traffic untouched."] # [doc = ""] # [doc = " Can track `MaxKeys` sources."] # [doc = ""] # [doc = " Each source has a key with type `K`."] # [doc = " [`IpAddrKey`](struct.IpAddrKey.html) is useful for this."] # [derive (Clone , Debug)] pub struct FairRateLimiter < K : Clone + Copy + Eq + Hash , const MAX_KEYS : usize > { tick_duration : Duration , sources_max : u32 , other_max : u32 , prng : Rand32 , sources_costs : RecentCosts , keys : HashMap < K , usize > , sources : Box < [Option < Source < K > >] > , other_costs : RecentCosts , }
};
}
