// Generated macro for wrap_worker_services (function)
macro_rules! Depcrate_workerwrap_worker_services {
() => {
// Module: crate::worker
// Provides: {"wrap_worker_services"}
// Dependencies: {}
fn wrap_worker_services (services : Vec < (usize , usize , BoxedServerService) >) -> Vec < WorkerService > { services . into_iter () . fold (Vec :: new () , | mut services , (idx , token , service) | { assert_eq ! (token , services . len ()) ; services . push (WorkerService { factory_idx : idx , service , status : WorkerServiceStatus :: Unavailable , }) ; services }) }
};
}
