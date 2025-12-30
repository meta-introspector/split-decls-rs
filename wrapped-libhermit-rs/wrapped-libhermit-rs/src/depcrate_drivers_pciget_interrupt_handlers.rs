// Generated macro for get_interrupt_handlers (function)
macro_rules! Depcrate_drivers_pciget_interrupt_handlers {
() => {
// Module: crate::drivers::pci
// Provides: {"get_interrupt_handlers"}
// Dependencies: {}
pub (crate) fn get_interrupt_handlers () -> HashMap < InterruptLine , InterruptHandlerQueue , RandomState > { let mut handlers : HashMap < InterruptLine , InterruptHandlerQueue , RandomState > = HashMap :: with_hasher (RandomState :: with_seeds (0 , 0 , 0 , 0)) ; for drv in PCI_DRIVERS . finalize () . iter () { let (irq_number , handler) = drv . get_interrupt_handler () ; if let Some (map) = handlers . get_mut (& irq_number) { map . push_back (handler) ; } else { let mut map : InterruptHandlerQueue = VecDeque :: new () ; map . push_back (handler) ; handlers . insert (irq_number , map) ; } } # [cfg (target_arch = "x86_64")] { use crate :: kernel :: serial :: get_serial_handler ; let (irq_number , handler) = get_serial_handler () ; if let Some (map) = handlers . get_mut (& irq_number) { map . push_back (handler) ; } else { let mut map : InterruptHandlerQueue = VecDeque :: new () ; map . push_back (handler) ; handlers . insert (irq_number , map) ; } } # [cfg (any (feature = "rtl8139" , feature = "virtio-net" ,))] if let Some (device) = NETWORK_DEVICE . lock () . as_ref () { handlers . entry (device . get_interrupt_number ()) . or_default () . push_back (crate :: executor :: network :: network_handler) ; } handlers }
};
}
