use serde::{Deserialize, Serialize};
use std::collections::HashMap;
extern "system" fn service_main(_len: u32, _args: *mut PWSTR) {
    let service: &Service = unsafe { &*(SERVICE_CONTEXT.read().unwrap().0 as *const Service) };
    *service.handle.write().unwrap() = unsafe {
        RegisterServiceCtrlHandlerExW(std::ptr::null(), Some(handler), service as *const _ as _)
    };
    service.set_state(State::StartPending);
    service.command(Command::Start);
    service.set_state(State::Running);
}
